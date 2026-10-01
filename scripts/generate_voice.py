import asyncio
import edge_tts
import os

TEXT = "It's time to drink water and get rest"
VOICE = "en-US-JennyNeural" # Natural and friendly AI neural voice
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OUTPUT_FILE = os.path.normpath(os.path.join(SCRIPT_DIR, "..", "assets", "voice_reminder.mp3"))

async def main():
    os.makedirs(os.path.dirname(OUTPUT_FILE), exist_ok=True)
    communicate = edge_tts.Communicate(TEXT, VOICE, rate="+0%", pitch="+0Hz")
    await communicate.save(OUTPUT_FILE)
    print(f"Generated AI voice saved to: {OUTPUT_FILE}")

if __name__ == "__main__":
    asyncio.run(main())
