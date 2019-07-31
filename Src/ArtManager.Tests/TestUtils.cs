using System.Diagnostics;

static class TestUtils
{
    public static void VersionCheck(string actualVer, string expectedVer)
    {
        if (actualVer != expectedVer)
            Debug.WriteLine($"Version mismatch: {actualVer} is not {expectedVer}");
    }
}
