# sea-orm-cli generate entity -u "postgres://root:root@localhost:5432/pos-schema" -o src/infrastructure/entities
$env = ".\.env"

if (Test-Path $env) {
    Get-Content $env | ForEach-Object {
        if (![string]::IsNullOrWhiteSpace($_) -and -not $_.StartsWith("#")) {
            $parts = $_.Split("=", 2)
            if ($parts.Length -eq 2) {
                $name = $parts[0].Trim()
                $value = $parts[1].Trim()

                Set-Item -Path Env:\$name -Value $value
                Write-Host "Loaded env variable: $name"
            }
        }
    }
} else {
    Write-Warning "'.env' file not found ad $env"
}


sea-orm-cli generate entity -u $env:DATABASE_URL -o generated