# torrent client with rust

inspired by
[tutorial](https://allenkim67.github.io/programming/2016/05/04/how-to-make-your-own-bittorrent-client.html)


<div align='center' styl>
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
- [x] get peers with tracker
- [ ] download
    - [x] tcp connection
    - [x] handshakes
    - [x] messages
    - [x] pieces
    - [x] managing connections and pieces



## to fix
- [ ] sometimes clients work in sequence instead of in paralllel (needs to be fixed) 
- [ ] add tests
- [ ] add documentation