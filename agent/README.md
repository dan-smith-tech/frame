1. Using `rpi-imager` flash the Raspberry PI 64bit OS to a microSD card, with SSH settings pre-enabled.

2. SSH into the Raspberry PI:

   ```bash
   ssh pi@<raspberry_pi_ip>
   ```

3. Crontab:

   ```bash
   0 2 * * * /home/dan/.virtualenvs/pimoroni/bin/python /home/dan/frame/agent/main.py
   ```
