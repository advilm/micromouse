void setup() {
  // put your setup code here, to run once:
  pinMode(14, OUTPUT);
}

void loop() {
  // put your main code here, to run repeatedly:
  digitalWrite(14, HIGH);
  delay(1000);
  digitalWrite(14, LOW);
  delay(1000);
}
