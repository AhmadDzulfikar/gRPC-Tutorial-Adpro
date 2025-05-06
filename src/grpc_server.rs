// NOTES: Kak asdos, saya nulis banyak komen buat saya belajar kak biar inget. 

// Apa Gunanya gRPC Server?

// gRPC Server adalah aplikasi atau layanan yang menyediakan API untuk berkomunikasi dengan klien. Server ini menangani permintaan (request) yang dikirim oleh klien, menjalankan logika bisnis yang diperlukan, dan mengirimkan kembali respons. Server ini biasanya **mengimplementasikan metode-metode yang didefinisikan dalam file .proto.
// Berikut adalah tujuan dan fungsi utama dari gRPC Server:
// Menangani Permintaan Klien: Server gRPC akan mendengarkan permintaan dari klien dan memberikan respons sesuai dengan metode yang didefinisikan dalam layanan.
// Proses dan Pengelolaan Data: Server biasanya berfungsi untuk menjalankan logika bisnis, mengelola data, dan memberikan respons yang sesuai (misalnya mengakses database atau memproses data).
// Implementasi Metode RPC: Server mengimplementasikan metode RPC yang didefinisikan dalam file .proto, yang dapat dipanggil oleh klien untuk mendapatkan data atau melakukan tindakan.
// Menyediakan Layanan Berbasis Streaming: Selain menangani permintaan sinkron, server gRPC juga dapat menangani komunikasi berbasis stream, seperti mengirimkan data dalam waktu nyata atau menangani banyak pesan.

use tonic::{transport::Server, Request, Response, Status}; // Import di rust yang berguna untuk ngeimpor berbagai komponen di dari tonic

pub mod services {  // ini gunanya buat mendefinisikan services di kode rust, Pub berguna agar model ini bakal bisa diakses di bagian kode lain di rust
    tonic::include_proto!("services");
}

//use berguna buat impor komponen (komponen yang dibutuhin buat bangun gRPC)

use services::{payment_service_server::{PaymentService, PaymentServiceServer}, PaymentRequest, PaymentResponse};

// Define si struct "MyPaymentService"
#[derive(Default)]
pub struct MyPaymentService {}

#[tonic::async_trait] // Biar fungsi dalam tonic dapat dijalankan 
impl PaymentService for MyPaymentService {
    async fn process_payment( // implementasi method RPC yang sudah didefinisikan dalam .proto untuk payment service
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Received payment request: {:?}", request);

        // Process the request and return a response
        // This example immediately returns a successful result for demonstration purposes
        Ok(Response::new(PaymentResponse { success: true })) // fungsi ini mengembalikan PaymentResponse
    }
}

#[tokio::main] // Menandai fungsi main sebagai asynchronus entry point di rust yang menggunakan tokio
async fn main() -> Result<(), Box<dyn std::error::Error>> { // mendefinisikan fungsi main sebagai asynchronus
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPaymentService::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .serve(addr)
        .await?;

    Ok(())
}


