1. What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?
   - RPC Unary:
     Deskripsi: Dalam metode ini, klien mengirimkan satu permintaan dan server memberikan satu respons, semuanya dalam satu siklus komunikasi.
     Skenario penggunaan: Cocok untuk operasi yang sederhana, seperti permintaan data atau pembaruan status yang tidak membutuhkan komunikasi berkelanjutan. Contohnya adalah pencarian atau pembaruan data pada basis data.
  
  - Streaming Server:
    Deskripsi: Dalam metode ini, klien mengirimkan satu permintaan dan server memberikan serangkaian respons secara terus-menerus. Server memulai pengiriman respons dan terus mengirim data hingga selesai atau sesuai instruksi.
    Skenario penggunaan: Cocok untuk aplikasi yang memerlukan respons yang panjang atau terus-menerus dari server, seperti riwayat transaksi atau pengambilan data besar secara streaming.

   - Streaming Dua Arah:
     Deskripsi: Baik klien maupun server dapat mengirimkan banyak pesan secara bersamaan dalam satu koneksi. Klien mengirimkan data terus-menerus, sementara server mengirimkan data balasan terus-menerus.
     Skenario penggunaan: Sering digunakan dalam aplikasi interaktif seperti aplikasi chat, di mana kedua pihak harus dapat mengirim dan menerima pesan secara terus-menerus.

2. What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?
   - Otentikasi: gRPC mendukung berbagai metode otentikasi seperti sertifikat TLS/SSL atau otentikasi berbasis token (seperti JWT) untuk memastikan bahwa hanya klien yang terverifikasi yang dapat mengakses layanan.
   - Otorisasi: Setelah otentikasi, gRPC dapat mengimplementasikan kontrol akses berbasis peran (RBAC) untuk membatasi akses ke metode atau data tertentu berdasarkan hak akses klien.
   - Enkripsi: gRPC menggunakan HTTP/2 yang memungkinkan enkripsi end-to-end menggunakan TLS (Transport Layer Security). Ini penting untuk mencegah penyadapan dan memastikan integritas data selama transmisi.

3. What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?
   - Menjaga aliran data yang lancar antara klien dan server, terutama ketika data terus mengalir dalam kedua arah.
   - Memastikan bahwa pengirim tidak kelebihan beban dalam mengirimkan data terlalu cepat (backpressure), serta penanganan buffering yang efisien untuk streaming data.
   - Dalam aplikasi chat, waktu respons yang cepat dan penggunaan sumber daya yang efisien adalah kunci. Ketika banyak klien mengakses server secara bersamaan, tantangannya adalah mengelola beban server.
   
4. What are the advantages and disadvantages of using the tokio_stream::wrappers::ReceiverStream for streaming responses in Rust gRPC services?
   - Keuntungan:
     - Asinkron dan skalabilitas, membantu memfasilitasi operasi non-blocking dan mendukung pengolahan banyak pesan secara bersamaan tanpa memblokir thread utama.
     - 'RecieverStream' bekerja dengan baik dalam ekosistem tokio, yang mendukung operasi non-blocking, ideal untuk aplikasi berkapasitas tinggi dan dengan latensi rendah.
       
   - Kerugian:
     - Kompleksitas: Penggunaan stream dapat menambah kompleksitas, terutama dalam hal pengelolaan kesalahan dan status koneksi.
     - Pengelolaan Memori: Jika tidak dikelola dengan benar, penggunaan buffer atau streaming dapat menyebabkan penggunaan memori yang lebih tinggi.

5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?
   - Modularisasi: Pisahkan kode ke dalam modul-modul terpisah berdasarkan fungsionalitasnya (misalnya, payment_service, transaction_service, chat_service). Ini memudahkan pemeliharaan dan pengembangan.
   - Penggunaan Traits: Gunakan traits untuk mendefinisikan perilaku yang dapat diimplementasikan oleh berbagai layanan. Ini akan membuat kode lebih fleksibel dan dapat digunakan kembali di tempat lain.

6. In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?
   - Verifikasi Pembayaran: Tambahkan logika untuk memverifikasi apakah pembayaran valid, seperti memeriksa saldo pengguna atau memastikan metode pembayaran sah.
   - Manajemen Status Pembayaran: Implementasikan manajemen status yang lebih canggih, seperti pending, gagal, dan berhasil, untuk menangani transaksi yang memerlukan waktu untuk diproses.
     
7. What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?
   - RPC dirancang untuk komunikasi yang cepat dan efisien, membuatnya ideal untuk aplikasi yang memerlukan latensi rendah dan kemampuan menangani volume data besar.
   - Implementasi skema yang ketat menggunakan Protocol Buffers dapat menambah kompleksitas dibandingkan dengan API REST berbasis JSON yang lebih fleksibel.
     
8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?
   - Keuntungan HTTP/2:
     - Multiplexing: HTTP/2 memungkinkan pengiriman beberapa permintaan dalam satu koneksi, mengurangi overhead.
     - Header Compression: Memperkecil ukuran header HTTP, meningkatkan efisiensi jaringan.

   - Kerugian HTTP/2:
     - Kompleksitas: HTTP/2 lebih kompleks dibandingkan HTTP/1, dan tidak semua server dan klien mendukungnya dengan baik.
       
9. How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?
    - Model Permintaan-Respons API REST:
      - Sinkron: Klien mengirimkan permintaan dan menunggu respons dari server.
      - Kurang Efisien untuk Real-time: Tidak cocok untuk aplikasi yang memerlukan respons real-time karena klien harus menunggu server merespons setiap permintaan.

    - Streaming Dua Arah:
      - Asinkron dan Real-Time: Baik klien dan server dapat mengirimkan data pada waktu yang bersamaan, membuatnya ideal untuk aplikasi seperti chat, game online, atau pembaruan data real-time.
      - 
10. What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?
- Protocol Buffers (gRPC):
    - Kecepatan dan Efisiensi: Lebih cepat dan lebih efisien dalam hal ukuran payload dan parsing dibandingkan dengan JSON.
    - Skema yang Ketat: Membutuhkan definisi skema yang ketat, yang memastikan bahwa data yang ditransfer memiliki struktur yang sudah ditentukan.

- JSON (API REST):
    - Fleksibilitas: JSON lebih fleksibel karena tidak memerlukan skema yang ketat, memungkinkan pengembangan yang lebih cepat dalam beberapa kasus.
    - Ukuran dan Kecepatan: Kurang efisien dalam hal ukuran dan kecepatan dibandingkan dengan Protocol Buffers, terutama ketika menangani data besar atau kompleks.
