Param(
    [string]$Name
)

# Define the source folder on the host machine
$sourceFolder = "C:\Users\svasista\Desktop\ENV\Windows-rust-driver-samples\target\debug\$Name"

# Define the destination folder on the VM
$destinationFolder = "C:\Users\User\Desktop\"

# Create a PowerShell Direct session to the VM
$vmName = "Windows 11 dev environment"
$vmUser = "User"
$vmPassword = ConvertTo-SecureString "password" -AsPlainText -Force
$credential = New-Object System.Management.Automation.PSCredential ($vmUser, $vmPassword)
$session = New-PSSession -VMName $vmName -Credential $credential

# Copy the folder from the host to the VM using the session
Copy-Item -Path $sourceFolder -Destination $destinationFolder -ToSession $session -Recurse -Force

# Close the session
Remove-PSSession $session