devgen.exe /add /bus ROOT /hardwareid "defective_echo"
pnputil.exe /add-driver .\defective_echo.inf /install

verifier

enable verifier