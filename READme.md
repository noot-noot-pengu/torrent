# torrent client with rust

sources

[tutorial in js](https://allenkim67.github.io/programming/2016/05/04/how-to-make-your-own-bittorrent-client.html)

[tutorial in go](https://blog.jse.li/posts/torrent/)

[torrent specification](https://wiki.theory.org/BitTorrentSpecification#piece:_.3Clen.3D0009.2BX.3E.3Cid.3D7.3E.3Cindex.3E.3Cbegin.3E.3Cblock.3E)

<div align='center' style>
    <img height='200' src='./logos/logo.svg'>
</div>


## notes
- copying the contents of a .torrent file doesn't work (idk why)
- copying the entire file then changing it's name is ok


## features
- NOTICE: this client only works with torrent files that have an announce url in them
- bencode encoder














## TODO
- [x] bencode decoder
- [x] parse torrent file
- [x] calculate info hash
- [x] get peers with tracker
- [ ] download
    - [x] tcp connection
    - [x] handshakes
    - [x] messages
    - [x] pieces
    - [x] managing connections and pieces
    - [x] downloading pieces
    - [ ] writing pieces to files
    - [ ] make it work with both single file and multiple files torrents


## extra features to implement
- [ ] remove clients when they error and refresh and add clients periodically
- [ ] recovery from all types of errors and disconnections (no unwraps in this app should be left)
- [ ] no reliance on announce (implementing DHT)
- [ ] magnet links
- [ ] when download is interrupted pick off where it started


## Errors:
- [ ] restart clients (clients keep breaking with): 
  - [ ] Resource temporarily unavailable (os error 11) ---> (message.rs line:107)
  - [ ] failed to fill whole buffer ---> (message.rs line:127)
  - [ ] Broken pipe (os error 32)
- [ ] receiving pieces in the wrong order "expected index: 44, got: 9"
- [x] insane CPU usage almost 100% (i5 8600k):
```rust
// this one was caused because of this code block which was ran everytime we received a PIECE message from a peer
// the CPU consumption is still a bit high and can be lowered but thats not a priority right now
// NOTE: this was discovered thanks to flamegraph profiler (https://github.com/flamegraph-rs/flamegraph)
progress.buf.splice((buf_begin as usize).., res_buf);
let mut file = std::fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open("buffers.txt")
    .unwrap();

writeln!(file, "{:?}", res_buf).unwrap();
```

## to fix
- [ ] some Peers responses come out as binary instead of text of Ip addresses (maybe change it to udp (https://www.bittorrent.org/beps/bep_0015.html))

it seems to be that getting peers from trackers is usually done with HTTP and not UDP, but some torrents return the response as an invalid bencode message
```
d8:completei31e10:downloadedi1262e10:incompletei7e8:intervali1914e12:min intervali60e5:peers228:`�Of�p̙ig:n�spа��{���\	|�0 ���H;NI��[='��"Χ���5����;Ѵ�����P�Fbe൒Fbj൒� jd]�Xh#�5����ᩖ�"#��?�AԳ<F�9�� ���<�g1Y��՘��'�I��4]"|�K�Y�[�kɜ25>i��D�����E^V��K#�+@O�'��T��Ӫ\U�-ۢ�V`�v3V���nN\��]GT���6:peers60:e
```
instead of 
```
d8:intervali900e5:peersld2:ip14:129.146.17.2084:porti6882eed2:ip13:178.92.140.174:porti20125eed2:ip13:76.193.65.2474:porti51413eed2:ip13:66.254.94.2064:porti50875eed2:ip14:151.40.222.1124:porti3702eed2:ip11:73.53.45.824:porti16881eed2:ip13:193.34.53.1724:porti5206eed2:ip13:87.101.92.1304:porti61636eed2:ip12:93.161.53.574:porti56251eed2:ip12:31.30.122.244:porti51413eed2:ip15:193.138.218.2504:porti19086eed2:ip14:146.70.166.2184:porti22433eed2:ip15:172.103.146.1304:porti21413eed2:ip14:67.168.246.2304:porti51413eed2:ip12:71.135.18.944:porti6881eed2:ip13:142.114.26.704:porti44625eed2:ip14:212.92.104.2164:porti42260eed2:ip12:66.102.91.484:porti6881eed2:ip12:91.64.163.894:porti51413eed2:ip12:185.40.4.1274:porti36046eed2:ip14:104.254.95.1144:porti51415eed2:ip13:193.163.71.364:porti22410eed2:ip12:93.51.17.1154:porti51413eed2:ip11:5.83.186.344:porti51413eed2:ip14:149.102.240.814:porti6881eed2:ip13:178.26.146.724:porti6881eed2:ip14:91.196.221.1224:porti50528eed2:ip15:185.200.116.1314:porti64378eed2:ip12:69.116.75.964:porti18460eed2:ip13:80.99.110.1884:porti51418eed2:ip13:62.45.139.2064:porti6881eed2:ip14:198.54.135.1974:porti6881eed2:ip14:138.201.155.874:porti51765eed2:ip13:50.40.237.1894:porti51413eed2:ip13:85.67.183.2414:porti6881eed2:ip12:142.93.68.634:porti1eed2:ip12:87.249.134.64:porti6881eed2:ip13:181.41.206.744:porti43067eed2:ip14:92.255.237.2254:porti48423eed2:ip13:198.48.168.654:porti60578eed2:ip13:77.174.164.374:porti51413eed2:ip14:185.239.193.444:porti12765eed2:ip11:144.2.65.954:porti40723eed2:ip14:104.152.208.274:porti60291eed2:ip12:81.171.17.994:porti43799eed2:ip13:122.199.31.284:porti6881eed2:ip14:213.232.87.2284:porti1eed2:ip13:62.213.82.1714:porti54878eed2:ip14:213.142.96.2374:porti60170eed2:ip12:92.161.65.134:porti51413eeee
```

could be that the binary data is bytes for ip.ip.ip.ip:port 6bytes total (this is correct)

- [ ] refactor and improve naming 
- [ ] divide code to more functions for better testing
- [ ] add connection resets in case of connection errors (using MSCP channels maybe)
- [x] send piece result to a writer thread with MSCP channels
- [ ] search for more clients regularly
- [ ] add documentation
- [ ] add all errors handling
- [ ] add all components tests