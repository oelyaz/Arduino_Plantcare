#include <ArduinoMqttClient.h>
#include <WiFi.h>
#include "arduino_secrets.h"

char ssid[] = SECRET_SSID;
char pass[] = SECRET_PASS;

WiFiClient wifiClient;
MqttClient mqttClient(wifiClient);

const char broker[] = "10.0.1.107";
int port = 1883;
const char topic[] = "uno_r4_plant_sensor";

HardwareSerial BridgeSerial(0);

void setup() {
  // debug serial
  Serial.begin(9600);

  Serial.println("Bridge Starting...");
  // bridge serial to ra4m1
  // RXD0 is GPIO44
  // TXD0 is GPIO43
  BridgeSerial.begin(9600, SERIAL_8N1, 44, 43);

  Serial.println("Connecting to WiFi and MQTT...");

  WiFi.begin(ssid, pass);
  delay(10000);

  while (!mqttClient.connect(broker, port)) {
    Serial.print("MQTT connection failed! Error code = ");
    Serial.println(mqttClient.connectError());
    delay(5000);
  }
  Serial.println("MQTT connected!");
}

void loop() {
  int message = 0;
  // keep mqtt alive
  mqttClient.poll();

  if (BridgeSerial.available() > 0) {

    message = BridgeSerial.read();
    Serial.println(message);

    if (message > 0) {

      mqttClient.beginMessage(topic);
      mqttClient.print(message);
      mqttClient.endMessage();
    }
  }
}
