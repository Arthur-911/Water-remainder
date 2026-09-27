import asyncio
import edge_tts
import os

TEXT = "It's time to drink water and get rest"
VOICE = "en-US-JennyNeural" # Natural and friendly AI neural voice
OUTPUT_FILE = r"C:\arthur_antigravity_shi\projects\remainder_app\assets\voice_reminder.mp3"

async def main():
    os.makedirs(os.path.dirname(OUTPUT_FILE), exist_ok=True)
    communicate = edge_tts.Communicate(TEXT, VOICE, rate="+0%", pitch="+0Hz")
    await communicate.save(OUTPUT_FILE)
    print(f"Generated AI voice saved to: {OUTPUT_FILE}")

if __name__ == "__main__":
    asyncio.run(main())
