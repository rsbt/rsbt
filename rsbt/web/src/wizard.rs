use rsbt_app::{AppError, Config};
use tracing;

const HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>rsbt Setup Wizard</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 600px;
            margin: 50px auto;
            padding: 20px;
            background: #f5f5f5;
        }
        .container {
            background: white;
            border-radius: 8px;
            padding: 30px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        h1 { color: #333; margin-bottom: 10px; }
        p { color: #666; line-height: 1.6; }
        .form-group { margin: 20px 0; }
        label { display: block; margin-bottom: 8px; font-weight: bold; }
        input[type="text"] {
            width: 100%;
            padding: 12px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-size: 16px;
        }
        button {
            background: #4CAF50;
            color: white;
            border: none;
            padding: 14px 28px;
            font-size: 16px;
            border-radius: 4px;
            cursor: pointer;
            margin-right: 10px;
        }
        button:hover { background: #45a049; }
        button.secondary {
            background: #666;
        }
        button.secondary:hover { background: #555; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Welcome to rsbt!</h1>
        <p>Let's set up your BitTorrent client. This wizard will help you configure rsbt.</p>
        
        <form id="setup-form">
            <div class="form-group">
                <label for="download-dir">Download Directory</label>
                <input type="text" id="download-dir" name="download-dir" value="{download_dir}">
            </div>
            
            <button type="submit">Complete Setup</button>
            <button type="button" class="secondary" onclick="location.href='/skip'">Skip</button>
        </form>
    </div>
    
    <script>
        document.getElementById('setup-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const downloadDir = document.getElementById('download-dir').value;
            
            const response = await fetch('/setup', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({ download_dir: downloadDir })
            });
            
            if (response.ok) {
                alert('Setup complete! You can now close this page.');
            } else {
                alert('Setup failed. Please try again.');
            }
        });
    </script>
</body>
</html>
"#;

pub fn serve_wizard(config: Config) -> Result<(), AppError> {
    use std::io::{Read, Write};
    use std::net::TcpListener;

    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).map_err(|e| AppError::Config(e.to_string()))?;

    tracing::info!("Web wizard available at http://{}/", addr);
    tracing::info!("Press Ctrl+C to cancel...");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mut buffer = [0; 1024];
        let _ = stream.read(&mut buffer).ok();

        let request = String::from_utf8_lossy(&buffer);

        if request.starts_with("GET / ") || request.starts_with("GET /index.html") {
            let html = HTML_TEMPLATE.replace(
                "{download_dir}",
                &config.download_dir().display().to_string(),
            );
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                html.len(),
                html
            );
            let _ = stream.write(response.as_bytes());
        } else if request.starts_with("POST /setup") {
            // Parse the body to get download directory
            let body_start = request.find("\r\n\r\n").map(|i| i + 4).unwrap_or(0);
            let body = &request[body_start..];

            if body.contains("download_dir") {
                // Extract download directory from JSON body
                if let Some(start) = body.find("download_dir") {
                    if let Some(colon) = body[start..].find(':') {
                        if let Some(quote_start) = body[start + colon..].find('"') {
                            if let Some(quote_end) =
                                body[start + colon + quote_start + 1..].find('"')
                            {
                                let download_dir = &body[start + colon + quote_start + 1
                                    ..start + colon + quote_start + 1 + quote_end];

                                // Create directories and config
                                let new_config =
                                    Config::new(Some(std::path::PathBuf::from(download_dir)));
                                new_config
                                    .ensure_dirs()
                                    .map_err(|e| AppError::Config(e.to_string()))?;

                                let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\":\"ok\"}";
                                let _ = stream.write(response.as_bytes());
                                continue;
                            }
                        }
                    }
                }
            }

            let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
            let _ = stream.write(response.as_bytes());
        } else if request.starts_with("GET /skip") || request.starts_with("GET /favicon") {
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            let _ = stream.write(response.as_bytes());
        } else {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
            let _ = stream.write(response.as_bytes());
        }
    }

    Ok(())
}
