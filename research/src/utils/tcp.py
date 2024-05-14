import socket

def tcp_client():
    # Create a TCP socket
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    # Set the server IP address and port number
    server_address = ('localhost', 8080)

    # Connect to the server
    client_socket.connect(server_address)

    while True:
        try:

            # Receive data from the server
            data = client_socket.recv(1024)
            print('Received:', data.decode())

        except ConnectionRefusedError:
            print('Connection refused. Make sure the server is running.')

