#include <arpa/inet.h>
#include <iostream>
#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

int main() {
  // Creazione del socket
  int serverSocket = socket(AF_INET, SOCK_STREAM, 0);
  if (serverSocket == -1) {
    std::cerr << "Errore durante la creazione del socket." << std::endl;
    return 1;
  }

  // Configurazione dell'indirizzo e della porta del server
  sockaddr_in serverAddr;
  serverAddr.sin_family = AF_INET;
  serverAddr.sin_port = htons(12345); // Numero di porta arbitrario
  serverAddr.sin_addr.s_addr = INADDR_ANY;

  // Associare il socket all'indirizzo e alla porta
  if (bind(serverSocket, (struct sockaddr *)&serverAddr, sizeof(serverAddr)) ==
      -1) {
    std::cerr << "Errore durante l'associazione del socket." << std::endl;
    close(serverSocket);
    return 1;
  }

  // Mettere il server in ascolto
  if (listen(serverSocket, 5) == -1) {
    std::cerr << "Errore durante la messa in ascolto del socket." << std::endl;
    close(serverSocket);
    return 1;
  }

  // Accettare le connessioni in ingresso
  sockaddr_in clientAddr;
  socklen_t clientAddrLen = sizeof(clientAddr);
  int clientSocket =
      accept(serverSocket, (struct sockaddr *)&clientAddr, &clientAddrLen);
  if (clientSocket == -1) {
    std::cerr << "Errore durante l'accettazione della connessione."
              << std::endl;
    close(serverSocket);
    return 1;
  }

  // Comunicazione con il client
  char buffer[1024];
  ssize_t bytesRead;
  while ((bytesRead = recv(clientSocket, buffer, sizeof(buffer), 0)) > 0) {
    // Processa i dati ricevuti...
    send(clientSocket, buffer, bytesRead, 0); // Invia una risposta al client
  }

  // Chiudi i socket
  close(clientSocket);
  close(serverSocket);

  return 0;
}
