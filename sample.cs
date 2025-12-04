class Program
{
    static void Main()
    {
        using (var port = new SerialPort("COM3", 9600))
        {
            port.Open();
            port.RtsEnable = true;
            port.DtrEnable = true;

            Console.WriteLine("Reading relays (Ctrl+C to exit):\n");
            
            while (true)
            {
                Console.Write($"\rRelay1: {(port.CtsHolding ? "CLOSED" : "OPEN  ")} | Relay2: {(port.DsrHolding ? "CLOSED" : "OPEN  ")}");
                Thread.Sleep(100);
            }
        }
    }
}
