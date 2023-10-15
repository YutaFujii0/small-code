# Use the official Python 3.9 image
FROM python:3.9

# Set environment variables
ENV PYTHONUNBUFFERED=1

# Set the working directory in the container
WORKDIR /app

# Copy the requirements.txt file and install the Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of the code into the container
COPY . .

# streamlit
EXPOSE 8501

# Run your code (replace this with whatever command you need)
CMD ["streamlit", "app.py"]
