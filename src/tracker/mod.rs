mod peers;

use peers::Peers;

use crate::torrent::Torrent;

pub fn get_peers(torrent_data: Torrent) -> Result<Peers, String> {
    Peers::get_peers(torrent_data)
}
