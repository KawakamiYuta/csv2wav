from socket import *
from struct import *
from ctypes import *
#from io import *
import io

class recvmcast():
    def __init__(self, localIp, mcastIp, mcastPort):
        self._sock = socket(AF_INET, SOCK_DGRAM)
        self._sock.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)
        self._sock.bind(('', mcastPort))
        self._sock.setsockopt(IPPROTO_IP, IP_ADD_MEMBERSHIP, inet_aton(mcastIp) + inet_aton(localIp))

    def recv():
        data = self._sock.recv(64 * 1024)
        return data

class wavfile_structure(BigEndianStructure):
    _fields_ = (
            ('ckID', c_uint32),
            ('cksize', c_uint32),
            ('WAVEID', c_uint32),
            )

class wavfile():
    def __init__(self, name):
        self._file = open(name, 'br')
        self._data = self._file.read();

    def hdr(self):
        hdr = io.BytesIO(self._data)
        hdr_type = wavfile_structure()
        hdr.readinto(hdr_type)
        print("%x" % hdr_type.ckID)
        #hdr_type.__str__

    def dump_as(self, name):
        return self.dump_as_range(name, 0, len(self._data))

    def dump_as_range(self, name, s, e):
        with open(name, 'wb') as f:
            wd = bytearray(self._data)
            f.write(wd[s:e])

if __name__ == "__main__":
    wfile = wavfile('music.wav')
    wfile.hdr()
    wfile.dump_as('aaa.wav')
#   udp = recvmcast("localhost", "XXX.XXX.XXX.XXX", 00000)
#    data = bytes(range(9))
#   udp.recv()
#    with open('sample.dat', 'bw') as f:
#        f.write(data)
# data = bytearray(bs)
# data[1] = xx
# data[5:]
