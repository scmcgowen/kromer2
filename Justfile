set shell := ["sh", "-c"]
set windows-shell := ["pwsh.exe", "-c"]

# Start MariaDB as a service
start-db:
    sudo net start MariaDB

# Stop the MariaDB service
stop-db:
    sudo net stop MariaDB
