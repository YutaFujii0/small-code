import java.util.ArrayList;
import java.util.LinkedList;

public class Album {
    private String name;
    private String artist;
    private SongList songs;

    public Album(String name, String artist) {
        this.name = name;
        this.artist = artist;
        this.songs = new SongList();
    }

    public boolean addSong(String title, double duration) {
        if (songs.findSong(title) != null) {
            return false;
        } else {
            songs.add(new Song(title, duration));
            return true;
        }
    }

    public boolean addToPlayList(int index, LinkedList<Song> playlist) {
        Song song = songs.findSong(index);
        if (song == null) {
            return false;
        } else {
            playlist.add(song);
            return true;
        }
    }

    public boolean addToPlayList(String title, LinkedList<Song> playlist) {
        Song song = songs.findSong(title);
        if (song != null) {
            playlist.add(song);
            return true;
        } else {
            return false;
        }
    }

    public class SongList {
        private ArrayList<Song> songs;

        public SongList() {
            songs = new ArrayList<Song>();
        }

        private boolean add(Song song) {
            if (findSong(song.getTitle()) != null) {
                return false;
            } else {
                songs.add(song);
                return true;
            }
        }

        private Song findSong(String title) {
            for (int i=0; i<songs.size(); i++) {
                if (songs.get(i).getTitle() == title) {
                    return songs.get(i);
                }
            }
            return null;
        }

        private Song findSong(int index) {
            if (index <= 0 || songs.size() < index) {
                return null;
            } else {
                return songs.get(index - 1);
            }
        }
    }
}
