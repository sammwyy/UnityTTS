using System.Runtime.InteropServices;
using UnityEngine;

sealed class TextToSpeech
{
#if !UNITY_EDITOR && (UNITY_IOS || UNITY_WEBGL)
    const string _dll = "__Internal";
#else
    const string _dll = "unitytts";
#endif

    [DllImport(_dll)] static extern void tts_say(int voice, string text);
}
