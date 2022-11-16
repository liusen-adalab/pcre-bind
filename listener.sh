port=$SERVER_PORT_PCRE
if [[ -z $port ]]
then
    port=5678
fi
echo "udp server listening on $port"
nc -nu -lp $port 