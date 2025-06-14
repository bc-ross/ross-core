import struct

import olefile


def extract_ole10native(bin_path):
    ole = olefile.OleFileIO(bin_path)
    if not ole.exists("\x01Ole10Native"):
        raise ValueError("No Ole10Native stream found!")

    stream = ole.openstream("\x01Ole10Native")
    data = stream.read()

    # Ole10Native format:
    #   4 bytes: total size
    #   null-terminated source path
    #   null-terminated temp path
    #   null-terminated file name
    #   file size (4 bytes)
    #   file content

    offset = 0
    total_size = struct.unpack("<I", data[offset : offset + 4])[0]
    offset += 4

    def read_cstring():
        nonlocal offset
        end = data.find(b"\x00", offset)
        s = data[offset:end]
        offset = end + 1
        return s

    source_path = read_cstring()
    temp_path = read_cstring()
    file_name = read_cstring()

    file_size = struct.unpack("<I", data[offset : offset + 4])[0]
    offset += 4

    file_content = data[offset : offset + file_size]

    return file_name.decode(), source_path.decode(), temp_path.decode(), file_content
