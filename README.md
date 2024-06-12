# Nexusmods Torrent Downloader

# WORK IN PROGRESS - COME BACK LATER

## Purpose
This program lets you download files from Nexusmods as torrents, getting around the speed limitation of a free account while also not putting any stress on NexusMods as a website.
This program can be used as the default program for openeing Vortex Mod Manager links. So when you click the "Vortex" Download button, this will try to download the torrent for the file into your Downloads Folder.

Pros (assuming you arent 1 of 100 people using it):
- Faster than normal downloads
- Files wont disappear if Mod or Release get deleted from Vortex

## Disclaimer

This is a curiosity and hobby project. In reality, the available torrent list will be incomplete (since i didnt download absolutely every file from NexusMods) and the speeds will also be limited unless lots of people picked up on this project and use it.

Using this is also against NexusMods TOS, so use at your own risk.

## Usage
You should get 3 things:
- Vortex Mod Manager for the files not available trough this program
- A capable not shady torrenting software, i use QBittorrent
- And of course, the tool it self

>On the subject of Vortex Modding tools to improve your experience, you should probably take a look at this: https://www.nexusmods.com/site/mods/288
>
>You can add this tool as a Mod Manager in NXM Proxy as well, just in case you have a use case for that.

1. Install NexusMods Torrent Downloader itself
   1. Install via whatever method is right for your OS
   2. Run the program once from the terminal (Powershell, Konsole, Kitty, cmd, whatever you have) so it can register itself as the handler for Vortex Downloads. Enter "nexusmods-torrent-downloader", press Enter and thats it.
2. Install your Torrenting software and set it up
   1. Install the software using whatever way is right for your OS
   2. (Optional) Some torrenting tools have a setting to watch a folder and automatically add torrent files from specific folders. If you dont want to manually add your torrents, this is the way
3. Test it by going to any Vortex modpage, and pressing the "Vortex" download button. If the tool is installed properly you will get a Desktop Notification telling you if it worked or not.

## Extra goodies
All the torrent files are in [./torrents](./torrents/), so if you only care about one or two, you can grab the files you need and dont use the tool at all.
> Structure is: ./torrents/files/GAME_FROM_VORTEX_URL/SECTION_FROM_VORTEX_URL/MOD_ID/FILE_ID

If you want to help out by seeding every and all files added to this tools list, there is a RSS Feed. This is also nice for if you just want to see what has been added.
> RSS Feed: https://raw.githubusercontent.com/ToasterUwU/nexusmods-torrent-downloader/main/torrents/feed.rss
