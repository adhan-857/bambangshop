# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [x] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [x] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [x] Commit: `Implement publish function in Program service and Program controller.`
    -   [x] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [x] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. Dalam *Observer design pattersn*, penerapan *interface* (atau *traits* dalam Rust) biasanya lebih disarankan karena memberikan fleksibilitas lebih tinggi dalam menambahkan berbagai tipe *subscriber*. Keberadaan *interface* memungkinkan kita untuk tidak perlu mengubah kode yang sudah ada, mengikuti *Open-close principle*. Namun, apabila hanya ada satu tipe *subscriber* dan dipastikan tidak akan bertambah, penggunaan *Single model struct* sudah memadai. Sehingga, jika pada kasus BambangShop hanya terdapat satu tipe *subscriber* dan dipastikan tidak ada penambahan tipe lain, maka *single model struct* sudah cukup untuk keperluan tersebut.
<br>

2. Menurut saya, penggunaan DashMap diperlukan untuk kasus ini. Salah satu kelebihan dari DashMap adalah kemampuannya untuk menyimpan key-value *pairs*, di mana *key* bersifat unik. Dengan DashMap, pencarian berdasarkan *key* menjadi lebih efisien. Dalam kasus ini, penggunaan DashMap memudahkan kita untuk menemukan *Subscriber* berdasarkan *url*-nya secara langsung. Sementara itu, apabila menggunakan *Vec*, proses pencarian harus dilakukan dengan cara iterasi untuk menemukan *Subscriber* yang tepat, yang mana dapat menghambat efisiensi dalam pengambilan dan *update* data.
<br>

3. Dalam pemrograman menggunakan Rust, DashMap digunakan dalam *Singleton pattern* untuk membantu mengelola operasi baca dan tulis secara bersamaan. Oleh karena itu, menurut saya penggunaan DashMap lebih baik jika dibandingkan dengan *HashMap* biasa, terutama dalam konteks aplikasi *multithreading*. Aplikasi yang kita kembangkan memerlukan kemampuan ini karena akan diakses oleh banyak *thread*. Dengan mengimplementasikan *Singleton pattern* seperti yang telah kita lakukan, kita bisa memastikan bahwa seluruh objek *subscriber* hanya berada dalam satu DashMap, karena *Singleton pattern* menjamin hanya ada satu *instance* dari objek tersebut. Ini mengoptimalkan manajemen sumber daya dalam aplikasi *multithreaded* dan memperkuat keamanan *thread*.

#### Reflection Publisher-2
1. Dalam *Model-View-Controller* (MVC) *compound pattern*, model mencakup penyimpanan data dan logika bisnis. Namun, pemisahan "Service" dan "Repository" dari model diperlukan untuk memenuhi *Single Responsibility Principle*. Dalam MVC, model sering kali terlalu kompleks karena mengelola *business logic*, validasi, data itu sendiri, dan operasi penyimpanan data. Dengan memindahkan *business logic* ke dalam "Service" dan operasi penyimpanan data ke "Repository", model kini hanya berperan sebagai representasi atau struktur dari kumpulan data tersebut. Pendekatan ini akan meningkatkan modularitas dan memudahkan pemeliharaan kode milik kita, karena setiap komponen akan memiliki fokus yang lebih spesifik.
<br>

2. Jika kita hanya menggunakan Model, maka kode untuk setiap Model akan menjadi besar. Hal ini akan mengakibatkan kode semakin kompleks dan sulit untuk dipahami. kode menjadi lebih sulit untuk dipahami dan di-*maintain*. Kondisi ini juga akan melanggar Single Responsibility Principle, di mana setiap komponen seharusnya hanya memiliki satu tanggung jawab. Misalnya, ketika ingin memodifikasi suatu logika, kita perlu memastikan bahwa perubahan tersebut tidak mempengaruhi komponen lain dalam aplikasi. Tanpa pemisahan yang jelas, hal ini dapat meningkatkan tingkat *coupling* antar komponen, sehingga mengurangi efektivitas dan efisiensi pemeliharaan kode.
<br>

3. Menurut saya, Postman adalah *tool* yang sangat bermanfaat untuk membuat dan mengembangkan suatu aplikasi, terutama dalam pengujian API. Postman memungkinkan saya untuk mengirim *HTTP request* dan memeriksa *response* yang diterima. Tentunya hal ini akan sangat membantu dalam proses *debugging* dan dalam peningkatan kualitas API pada *project* yang akan saya lakukan di kemudian hari.

#### Reflection Publisher-3
1. Pada kasus, kita menerapkan variasi *Push Model* dari *Observer pattern*. Penyebabnya adalah setiap kali terjadi penambahan atau penghapusan produk, `notify` *method* pada `NotificationService` dijalankan. Hal ini menunjukkan bahwa *publisher* memiliki kemampuan untuk secara proaktif mengirim notifikasi dan data terkait kepada *subscriber*-nya. *Subscriber* menerima notifikasi ini secara otomatis tanpa perlu membuat *request* terlebih dahulu.
<br>

2. Jika kita menerapkan variasi *Pull Model* dari *Observer pattern* pada tutorial ini, maka *subscriber* yang akan menentukan kapan mereka mengambil data atau menerima notifikasi dari *publisher*. Keuntungan dari pendekatan ini adalah *subscriber* dapat mengontrol waktu pengambilan data, sehingga tidak terganggu oleh notifikasi pada waktu yang tidak diinginkan. Selain itu, *subscriber* juga dapat memilih data spesifik yang ingin mereka ambil. Namun, kelemahannya adalah *subscriber* tidak akan dapat menerima data atau notifikasi secara langsung atau segera saat terjadi penambahan atau penghapusan produk, yang bisa menghambat responsivitas dalam beberapa skenario.
<br>

3. Jika kita tidak menggunakan *multi-threading*, maka aplikasi akan mengirim notifikasi secara berurutan satu per satu. Kondisi ini dapat menyebabkan proses pengiriman notifikasi menjadi lambat, terutama bila jumlah *subscriber* banyak. Meskipun program tidak akan mengalami error, kecepatan proses notifikasi bisa terhambat secara signifikan.