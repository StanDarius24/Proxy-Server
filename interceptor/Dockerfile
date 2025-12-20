FROM python:3.12-slim

WORKDIR /app

COPY app.py ./

RUN pip install flask --no-cache-dir

EXPOSE 80

CMD ["python", "app.py"]
