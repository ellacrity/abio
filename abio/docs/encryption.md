The following is a very small "guide" that I decided to put together that describes
a few common tasks and how I personally like to perform them. These little "tricks"
can make your life much easier and [code]gpg[/code] will hopefully seem less daunting (it has a
LOT of options!!).

[center]Prelude[/center]

[center]NOTICE: DO NOT RUN ANY COMMAND IN THIS "GUIDE" USING `sudo` OR AS `root` UNLESS YOU KNOW EXACTLY WHAT YOU ARE DOING. DOING SO MAY RESULT IN DESTRUCTIVE ACTIONS.[/center]

[title]Pre-Conditions[/title]

[list]
- Your host OS is some Linux distribution
- Beginners should opt for something Debian-based, such as Ubuntu (don't use Debian itself if you are unfamiliar with Linux).
- Intermediate to advanced users can use whatever they like (people still will typically recommend Debian LTS).
- Rudimentary knowledge of how your OS works
  - Don't panic! You do not need to know everything. You can do an internet search to fill in any knowledege gaps you need :)
- Be familiar with and use your terminal aka shell (or "console").
  - Most resources assume you are using [code]bash[/code], which is POSIX-compliant. Unless you know what you are doing, you should opt to use [code]bash[/code], [code]zsh[/code] or another POSIX-compliant shell.
- Install a utility to manage your clipboard, such as [code]xsel[/code] or [code]xclip[/code] (I prefer [code]xsel[/code] and will be using that in my examples).
  - These programs allow you to quickly and painlessly pipe the contents of your clipboard to other programs, like [code]gpg[/code]!
[/list]
  
[b]NOTE: In the event that your machine is not using X11 for your compositor, then you you simply need to use the equivalent program for "Wayland", such as [code]wl-clipboard[/code].
  - You should be able to find this information in your system settings or by checking the output from [code]echo $XDG_SESSION_TYPE[/code].
[/b]

[center]GPG Guide[/center]

[title]Generating Your Secret Key Pair[/title]

To start, you should create a new key using the largest available key size (4096 bits). Note that if you use the default, you will probably end up with a 3072-bit key, which is still sufficient, but not recommended. Go the extra mile and make your key at least a bit more secure.

Create your secret key pair by following these steps:

Enter the following command into your terminal:

[code]gpg --full-generate-key --expert[/code]
[list]
1. Select (1) to use RSA for both auth and signing
2. Enter [code]4096[/code] to set your key size to 4096 bits (the maximum allowed size).
3. Enter [code]4096[/code] again if prompted to select subkey size. If you are not asked again, skip to the next step.
4. Set the expiration for the key. You can use [code]0[/code], so it does not expire. You can revoke the key later, at an appropriate time.
[/list]

[center]Basic Setup of [code]$HOME/.gnupg/[/code] Config Files[/center]



You can configure your "default" settings for GPG (found in [code]/home/<username>/.gnupg/gpg.conf[/code]) and the GPG background daemon (gpg-agent.conf) can make your life a lot easier, but I rarely see these things mentioned in guides.

The official documentation related to these files and the programs that make up [code]gpg[/code] can be quite technical and difficult to understand, even for someone with experience. I do not recommend modifying any settings that you do not understand. unless you have an in-depth knowledge of how the pieces all fit together. If you really want to learn more about this stuff, you can find the help files (man pages) on your machine using [code]apropos <search term>[/code] to get a list of related manual pages.

This part it rarely mentioned in guides that I see, but it is actually quite useful! You can make your life easier and avoid passing the same, repititious flags into the [code]gpg[/code] CLI. The first time you run a [code]gpg[/code] command, your $HOME directory should be created and a trust DB (as well as a bunch of other things) will be created.

This is optional, and arguably preference, but you can automatically enable `ascii` armor so you can easily work with `.asc` files, which will contain base64-encoded data rather than binary data. That way, you can see the PGP message on disk or what not.

To enable this setting:
[code]echo "armor" >> ~/.gnupg/gpg.conf[/code] (this will enable ascii armor for messages/key exports/etc.). You can simply pass in the `--armor` flag in the terminal, instead.

Another thing I like to see in all output is the keygrip (again this is optional and personal preference).

This one can be enabled using:
[code]echo "with-keygrip" >> ~/.gnupg/gpg.conf[/code]

[title]Importing a New PGP Key From User / Vendor[/title]

1. Select their key and copy it to your clipboard
2. Using your terminal, you can now do: [code]xsel --clipboard | gpg --import[/code]
    i. This should automatically add the GPG key to your keyring.
    ii. This can be shortened to the equivalent command: [code]xsel -b | gpg --import[/code]
3. Type [code]gpg -k[/code] to list the keys in your keyring. You should see the newly added one.
    i. Also, typing [code]gpg -K[/code] (with a capital K) will list all of your own secret keys.

[title]Creating Encrypted Messages[/title]
This can be done in a variety of ways, and you can output the result to a file, your console, or wherever you want to pipe the output to.
The following will encrypt your clipboard contents and output them to a file that you can view:
[code]xsel -b | gpg --decrypt --armor > filename[/code]
