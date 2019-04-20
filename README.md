# ServerMonitor
A server monitor, collecting the player count from multiple servers at once - written in Rust.  
  
## Getting started
* Install [Rust](https://www.rust-lang.org/):  
  - [Download for Windows](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)
  - Linux: `curl https://sh.rustup.rs -sSf | sh`
* Clone the Repository: `git clone https://github.com/Xijezu/ServerMonitor.git`
* Open the ServerMonitor directory
* Run `cargo build --release`  
The executable is located in `/target/release/` as `server_monitor`(.exe)
* Create a servers.json containing a list of servers to connect to (see example below) and a monitor.ini with your configuration (see example below)

## Using MySQL for logging
First you need to create a new database named "Log".  
To create the table for logging, use this query:
```sql
DROP TABLE IF EXISTS `ServerMonitor`;
CREATE TABLE `ServerMonitor` (
  `name` varchar(50) COLLATE utf8mb4_unicode_ci NOT NULL,
  `scrape_date` timestamp NOT NULL DEFAULT '0000-00-00 00:00:00' ON UPDATE current_timestamp(),
  `user_count` int(11) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
```
If you want to get a pretty clean graph of the log, I'd highly recommend looking into [Python Server Log Plotting](https://github.com/Jynsaillar/python_server_log_plotting) by Jynsaillar.

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
database.connection = "mysql://username:password@ip_address:port"
# Example: database.connection = "mysql://root:password@localhost:3306"
```