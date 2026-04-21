$cutoff = (Get-Date).AddMinutes(-20)
Get-WinEvent -LogName Application -MaxEvents 600 -ErrorAction SilentlyContinue |
    Where-Object {
        $_.TimeCreated -gt $cutoff -and
        ($_.ProviderName -eq 'Application Error' -or
         $_.ProviderName -eq 'Application Hang' -or
         $_.ProviderName -eq '.NET Runtime' -or
         $_.ProviderName -eq 'Windows Error Reporting')
    } |
    Select-Object -First 10 TimeCreated, ProviderName, Id, LevelDisplayName, Message |
    Format-List
