/*
 * 
 * All the resources for this project: http://randomnerdtutorials.com/
 * Modified by Rui Santos
 * 
 * Created by FILIPEFLOP
 * 
 */
 
#include <SPI.h>
#include <MFRC522.h>

#include <Wire.h>


#define SS_PIN 10
#define RST_PIN 9
MFRC522 mfrc522(SS_PIN, RST_PIN);   // Create MFRC522 instance.
 
void setup() 
{
  Serial.begin(9600);   // Initiate a serial communication
  SPI.begin();      // Initiate  SPI bus
  mfrc522.PCD_Init();   // Initiate MFRC522
}

String lookup_card(){
  // Look for new cards
  if ( ! mfrc522.PICC_IsNewCardPresent()) 
  {
    return "";
  }
  // Select one of the cards
  if ( ! mfrc522.PICC_ReadCardSerial()) 
  {
    return "";
  }
  return mfrc522.uid.uidByte;
}

const String base = "0123456789ABCDEF";

void loop() 
{
  String content = lookup_card();

  for (int i=0; content[i] != 0; i++) {
    Serial.print(base[content[i]>>4]);
    Serial.print(base[content[i]&&15]);
  }
  if (content == ""){
    return;
  } else {
    Serial.println("");
  }
  delay(500);
} 
