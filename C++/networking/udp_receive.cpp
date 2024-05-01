#include <iostream>
#include <cstring>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

using namespace std;

int main() {
  int sockfd = socket(AF_INET, SOCK_DGRAM, 0);
  if (sockfd < 0) {
    cerr << "Errore creazione socket." << endl;
    return 1;
  }

  struct sockaddr_in localAddr;
  memset(&localAddr, 0, sizeof(localAddr));
  localAddr.sin_family = AF_INET;
  localAddr.sin_addr = htonl(INADDR_ANY);
  localAddr.sin_port = htons(12345);
  
  if (bind(sockfd, (struct sockaddr*)&localAddr, sizeof(localAddr)) < 0) {
    cerr << "Errore associazione indirizzo" << endl;
    return 1;
  }

  const int BUFFER_SIZE = 1024;
  char buffer[BUFFER_SIZE];

  struct sockaddr_in remoteAddr;
  socklen_t remoteAddrLen = sizeof(remoteAddr);
}