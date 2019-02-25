New-Item -ItemType Directory -Force -Path .\oanc
Set-Location .\oanc
Invoke-WebRequest "http://www.anc.org/OANC/OANC-1.0.1-UTF8.zip" -OutFile "OANC-1.0.1-UTF8.zip"
Expand-Archive "OANC-1.0.1-UTF8.zip" -DestinationPath ".\OANC-1.0.1-UTF8"