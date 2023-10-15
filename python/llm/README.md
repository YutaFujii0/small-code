Reference

https://zenn.dev/ml_bear/books/d1f060a3f166a5/viewer/0e8fe3


```bash
cd python/llm

docker build -t llm_demo .

docker run -it --rm \
--name llm_demo \
-p 8501:8501 \
--mount type=bind,source="$(pwd)",target=/app \
llm_demo /bin/bash
```