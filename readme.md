# UE5 Project Tools

![Unreal Engine 5 Logo](https://cdn2.unrealengine.com/ue-logotype-2023-vertical-white-1686x2048-bbfded26daa7.png)

Утилита для резервного копирования и восстановления проектов Unreal Engine 5 с поддержкой (s3 bucket) Yandex Object Storage.

## 📌 Возможности


- 📦 Создание резервных копий проектов:
    - Сжатие в архив 7z
    - Загрузка в Yandex Object Storage
- ♻ Восстановление проектов из бэкапа:
    - Скачивание из облачного хранилища
    - Распаковка архива

## 🛠 Установка

1. Убедитесь, что установлен [Rust](https://www.rust-lang.org/tools/install)
2. Соберите проект:

```bash
cargo build --release