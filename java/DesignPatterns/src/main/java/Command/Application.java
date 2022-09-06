package Command;

import java.util.LinkedList;
import java.util.List;

public class Application {

    private static List<Document> docRepo = new LinkedList<Document>();

    public void addDoc(Document doc) {
        docRepo.add(doc);
    }
}
