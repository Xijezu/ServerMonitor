# ServerMonitor
A server monitor, collecting the player count from multiple servers at once - written in Rust.
  

## Sample JSON: servers.json
```json
{
    "region": [
        {
            "name": "Region 1",
            "auth": "127.0.0.1",
            "servers": [
                {
                    "name": "Server 1",
                    "ip": "127.0.0.1",
                    "port": 1337,
                    "players": 0,
                    "packet_version": 2
                },
                {
                    "name": "Server 2",
                    "ip": "127.0.0.2",
                    "port": 1337,
                    "players": 0,
                    "packet_version": 2
                }
            ]
        },
        {
            "name": "Region 2",
            "auth": "127.0.0.3",
            "servers": [
                {
                    "name": "Server 3",
                    "ip": "127.0.0.4",
                    "port": 1337,
                    "players": 0,
                    "packet_version": 1
                }
            ]
        }
    ]
}
```  
  
  ## Sample Config: monitor.ini
  ```ini
  [monitor]
server.update_timer = 1 # In minutes
server.save_path = "/var/www/test/sample.json"
```