// NOTES: Kak asdos, saya nulis banyak komen buat saya belajar kak biar inget. 

// Apa Gunanya gRPC Server?

// gRPC Server adalah aplikasi atau layanan yang menyediakan API untuk berkomunikasi dengan klien. Server ini menangani permintaan (request) yang dikirim oleh klien, menjalankan logika bisnis yang diperlukan, dan mengirimkan kembali respons. Server ini biasanya **mengimplementasikan metode-metode yang didefinisikan dalam file .proto.
// Berikut adalah tujuan dan fungsi utama dari gRPC Server:
// Menangani Permintaan Klien: Server gRPC akan mendengarkan permintaan dari klien dan memberikan respons sesuai dengan metode yang didefinisikan dalam layanan.
// Proses dan Pengelolaan Data: Server biasanya berfungsi untuk menjalankan logika bisnis, mengelola data, dan memberikan respons yang sesuai (misalnya mengakses database atau memproses data).
// Implementasi Metode RPC: Server mengimplementasikan metode RPC yang didefinisikan dalam file .proto, yang dapat dipanggil oleh klien untuk mendapatkan data atau melakukan tindakan.
// Menyediakan Layanan Berbasis Streaming: Selain menangani permintaan sinkron, server gRPC juga dapat menangani komunikasi berbasis stream, seperti mengirimkan data dalam waktu nyata atau menangani banyak pesan.

use tonic::{transport::Server, Request, Response, Status}; // Import di rust yang berguna untuk ngeimpor berbagai komponen di dari tonic
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc::{Receiver, Sender};

pub mod services {  // ini gunanya buat mendefinisikan services di kode rust, Pub berguna agar model ini bakal bisa diakses di bagian kode lain di rust
    tonic::include_proto!("services");
}

//use berguna buat impor komponen (komponen yang dibutuhin buat bangun gRPC)
// use necessary services
use services::{payment_service_server::{PaymentService, PaymentServiceServer}, PaymentRequest, PaymentResponse,
                transaction_service_server::{TransactionService, TransactionServiceServer}, TransactionRequest, TransactionResponse};


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

#[derive(Default)]
pub struct MyTransactionService {}

#[tonic::async_trait]
impl TransactionService for MyTransactionService {
    type GetTransactionHistoryStream = ReceiverStream<Result<TransactionResponse, Status>>;

    async fn get_transaction_history(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<Self::GetTransactionHistoryStream>, Status> {
        println!("Received transaction history request: {:?}", request);

        let (tx, rx): (Sender<Result<TransactionResponse, Status>>, Receiver<Result<TransactionResponse, Status>>) = mpsc::channel(4);

        tokio::spawn(async move {
            for i in 0..30 { // Simulate sending 30 transaction records
                if tx.send(Ok(TransactionResponse {
                    transaction_id: format!("T{}", i),
                    status: "Completed".to_string(),
                    amount: 100.0,
                    timestamp: "2022-01-01T12:00:00Z".to_string(),
                })).await.is_err() {
                    break;
                }
                if i % 10 == 0 {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}


#[tokio::main] // Menandai fungsi main sebagai asynchronus entry point di rust yang menggunakan tokio
async fn main() -> Result<(), Box<dyn std::error::Error>> { // mendefinisikan fungsi main sebagai asynchronus
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPaymentService::default();
    let transaction_service = MyTransactionService::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .add_service(TransactionServiceServer::new(transaction_service))
        .serve(addr)
        .await?;

    Ok(())
}


