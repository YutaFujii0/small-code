package com.yuta;

import java.io.FileOutputStream;
import java.io.IOException;
import java.io.RandomAccessFile;
import java.nio.ByteBuffer;
import java.nio.channels.FileChannel;

public class Main {
    public static void main(String[] args) {
        try (FileOutputStream binFile = new FileOutputStream("data.dat");
             FileChannel binChannel = binFile.getChannel()) {

            byte[] outputBytes = "YutaFujiiHello World!Aaこaんaにaち🚀a".getBytes();
            // wrap
//            ByteBuffer buffer = ByteBuffer.wrap(outputBytes);

            // or allocate & put (don't forget to flip())
            ByteBuffer buffer = ByteBuffer.allocate(outputBytes.length + 10);
            buffer.put(outputBytes);
//            int pos = buffer.position();
//            buffer.position(pos + 1);
            buffer.flip();

            int numBytes = binChannel.write(buffer);
            System.out.println("numBytes written was: " + numBytes);

            ByteBuffer intBuffer = ByteBuffer.allocate(Integer.BYTES);
            intBuffer.putInt(1509949439);
            // intBuffer.putInt(245);
            intBuffer.flip();
            numBytes = binChannel.write(intBuffer);
            System.out.println("numBytes written was: " + numBytes);

            intBuffer.flip();
            intBuffer.putInt(45344);
            intBuffer.flip();
            numBytes = binChannel.write(intBuffer);
            System.out.println("numBytes written was: " + numBytes);

            RandomAccessFile ra = new RandomAccessFile("data.dat", "rwd");
            FileChannel channel = ra.getChannel();
            outputBytes[0] = 'a';
            outputBytes[1] = 'b';
            buffer.flip();

            ByteBuffer buffer2 = ByteBuffer.allocate(outputBytes.length + 10);
            buffer2.put("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".getBytes());
            buffer2.clear();
            long numBytesRead = channel.read(buffer2);
            System.out.println("numBytes read was: " + numBytesRead);

            // this works fine if outputBytes is wrapped in buffer (ByteBuffer.wrap(outputBytes))
            System.out.println("outputBytes = " + new String(outputBytes));

            // prefer to use hasArray()
            if (buffer.hasArray()) {
                System.out.println("byte buffer = " + new String(buffer.array()));
            }


//            binFile.write(data);
//        try {
////            FileInputStream file = new FileInputStream("data.txt");
////            FileChannel channel = file.getChannel();
//            Path dataPath = FileSystems.getDefault().getPath("data.txt");
////            Files.write(dataPath, "\nLine 4".getBytes("UTF-8"), StandardOpenOption.APPEND);
//
//            List<String> lines = Files.readAllLines(dataPath);
//            for (String line : lines) {
//                System.out.println(line);
//            }

        } catch (IOException e) {
            System.out.println("IOException");
            e.printStackTrace();
        }
    }
}
