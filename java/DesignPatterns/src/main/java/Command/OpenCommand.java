package Command;

public class OpenCommand implements Command {

    private Application app;

    public OpenCommand(Application app) {
        this.app = app;
    }

    @Override
    public void execute() {
        Document doc = new Document();
        app.addDoc(doc);
        doc.open();
    }
}
