FROM amazoncorretto:11
#VOLUME /tmp
WORKDIR /app
COPY . /app
CMD bash -c "javac /app/src/Main.java && java -cp /app/src Main < /app/input"