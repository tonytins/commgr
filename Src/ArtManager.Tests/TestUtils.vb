Module TestUtils

    Sub VersionCheck(actualVer As String, expectedVer As String)

        If actualVer IsNot expectedVer Then
            Debug.WriteLine($"Version mismatch: {actualVer} is not {expectedVer}")
        End If

    End Sub

End Module
