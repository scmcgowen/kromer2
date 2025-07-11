set shell := ["sh", "-c"]
set windows-shell := ["pwsh.exe", "-c"]

# Start Postgres as a service
start-db:
    sudo net start postgresql-x64-17

# Stop the Postgres service
stop-db:
    sudo net stop postgresql-x64-17
