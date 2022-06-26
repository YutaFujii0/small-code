import java.util.ArrayList;
import java.util.LinkedList;

public class Album {
    private String name;
    private String artist;
    private ArrayList<Song> songs;

    public Album(String name, String artist) {
        this.name = name;
        this.artist = artist;
        this.songs = new ArrayList<Song>();
    }

    public boolean addSong(String title, double duration) {
        if (findSong(title) != null) {
            return false;
        } else {
            songs.add(new Song(title, duration));
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

    public boolean addToPlayList(int index, LinkedList<Song> playlist) {
        if (index <= 0 || songs.size() < index) {
            return false;
        } else {
            playlist.add(songs.get(index - 1));
            return true;
        }
    }

    public boolean addToPlayList(String title, LinkedList<Song> playlist) {
        Song song = findSong(title);
        if (song != null) {
            playlist.add(song);
            return true;
        } else {
            return false;
        }
    }
}
