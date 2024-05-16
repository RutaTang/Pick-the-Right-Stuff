def read_until_separator(client_socket):
    SEPARATOR = 0x03
    content_buffer = bytearray()

    while True:
        chunk = client_socket.recv(1)
        if len(chunk) == 0:
            raise Exception("Stream closed")

        if chunk[0] == SEPARATOR:
            break
        else:
            content_buffer.append(chunk[0])

    return content_buffer


def write_to_stream(client_socket, data, end):
    data = data.encode()
    if end:
        data += bytes([0x03])
    client_socket.sendall(data)
