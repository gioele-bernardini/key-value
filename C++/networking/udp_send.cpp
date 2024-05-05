#include <iostream>
#include <cstring>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

using namespace std;

int main() {
  // Creazione della Socket
  int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
  if (sockfd < 0) {
    cerr << "Errore nella creazione socket." << endl;
    return 1;
  }

  // Indirizzo del destinatario
  struct sockaddr_in destAddr;
  memset(&destAddr, 0, sizeof(destAddr));
  destAddr.sin_family = AF_INET;
  destAddr.sin_port = htons(12345);
  inet_pton(AF_INET, "192.168.1.100", &destAddr.sin_addr);

  // Dati del pacchetto da inviare
  const char* buffer = "Hello, World!";
  int bufferLength = strlen(buffer);

  // Invio del pacchetto
  int bytesSent = sendto(
    sockfd, 
    buffer, bufferLength,
    0, 
    (struct sockaddr*)&destAddr, 
    sizeof(destAddr)
  );

  if (bytesSent < 0) {
    cerr << "Errore nell'invio pacchetto." << endl;
    return 1;
  }

  cout << "Pacchetto inviato correttamente" << endl;

  close(sockfd);

  return 0;
}