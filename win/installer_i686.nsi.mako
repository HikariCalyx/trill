<%
import os
import semver
import toml

with open(os.path.join(os.path.dirname(__file__), "..", "tango", "Cargo.toml")) as f:
    cargo_toml = toml.load(f)


version = semver.Version.parse(cargo_toml["package"]["version"])

%>!define NAME "Trill"
!define REGPATH_UNINSTSUBKEY "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\<%text>$</%text>{NAME}"

LoadLanguageFile "<%text>$</%text>{NSISDIR}\Contrib\Language files\English.nlf"
LoadLanguageFile "<%text>$</%text>{NSISDIR}\Contrib\Language files\Japanese.nlf"
LoadLanguageFile "<%text>$</%text>{NSISDIR}\Contrib\Language files\TradChinese.nlf"
LoadLanguageFile "<%text>$</%text>{NSISDIR}\Contrib\Language files\SimpChinese.nlf"

Name "<%text>$</%text>{NAME}"
Icon "icon.ico"
OutFile "installer.exe"

VIProductVersion "${version.major}.${version.minor}.${version.patch}.0"
VIAddVersionKey "ProductName" "<%text>$</%text>{NAME}"
VIAddVersionKey "FileVersion" "${version.major}.${version.minor}.${version.patch}.0"
VIAddVersionKey "FileDescription" "Trill Installer"
VIAddVersionKey "LegalCopyright" "© Hikari Calyx Tech"

SetCompressor /solid /final lzma
Unicode true
RequestExecutionLevel user
AutoCloseWindow true
ShowInstDetails nevershow
ShowUninstDetails nevershow
BrandingText " "
ChangeUI all "<%text>$</%text>{NSISDIR}\\Contrib\\UIs\\sdbarker_tiny.exe"

InstallDir ""
InstallDirRegKey HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "UninstallString"

!include LogicLib.nsh
!include WinCore.nsh
!include FileFunc.nsh

Function .onInit
    SetShellVarContext Current

    <%text>$</%text>{If} $INSTDIR == ""
        GetKnownFolderPath $INSTDIR <%text>$</%text>{FOLDERID_UserProgramFiles}
        StrCmp $INSTDIR "" 0 +2
        StrCpy $INSTDIR "$LocalAppData\\Programs"
        StrCpy $INSTDIR "$INSTDIR\\$(^Name)"
    <%text>$</%text>{EndIf}

    ExecWait '"$INSTDIR\\Uninstall Trill.exe" /S'
FunctionEnd

Function un.onInit
    SetShellVarContext Current
FunctionEnd

LangString MessageDeleteConfig <%text>$</%text>{LANG_ENGLISH} "Would you also like to delete configuration settings?"
LangString MessageDeleteConfig <%text>$</%text>{LANG_JAPANESE} "コンフィギュレーション設定も削除しますか？"
LangString MessageDeleteConfig <%text>$</%text>{LANG_SIMPCHINESE} "您是否也想删除配置设置？"
LangString MessageDeleteConfig <%text>$</%text>{LANG_TRADCHINESE} "您是否也想刪除配置設置？"

Function un.onGUIInit
    MessageBox MB_YESNO "$(MessageDeleteConfig)" /SD IDNO IDYES true IDNO false
    true:
        Delete "$APPDATA\\Trill\\config\\config.json"
    false:
FunctionEnd

Function .onInstSuccess
    Exec "$INSTDIR\\trill.exe"
FunctionEnd

Section
    SetDetailsPrint none
    SetOutPath $INSTDIR
    File "libstdc++-6.dll"
    File "libEGL.dll"
    File "libGLESv2.dll"
    File "libgcc_s_dw2-1.dll"
    File "libwinpthread-1.dll"
    File "ffmpeg.exe"
    File "trill.exe"
    WriteUninstaller "$INSTDIR\\uninstall.exe"
    WriteRegStr HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "DisplayName" "<%text>$</%text>{NAME}"
    WriteRegStr HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "DisplayIcon" "$INSTDIR\\trill.exe,0"
    WriteRegStr HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "Publisher" "The Trill Developers"
    WriteRegStr HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "InstallLocation" "$INSTDIR"

    IntFmt $0 "0x%08X" "{version.major}"
    WriteRegDWORD HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "VersionMajor" "$0"

    IntFmt $0 "0x%08X" "{version.minor}"
    WriteRegDWORD HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "VersionMinor" "$0"

    WriteRegStr HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "DisplayVersion" "{version}"
    WriteRegStr HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "UninstallString" '"$INSTDIR\\uninstall.exe"'
    WriteRegStr HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "QuietUninstallString" '"$INSTDIR\\uninstall.exe" /S'

    <%text>$</%text>{GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
    IntFmt $0 "0x%08X" $0
    WriteRegDWORD HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "EstimatedSize" "$0"

    WriteRegDWORD HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "NoModify" 1
    WriteRegDWORD HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}" "NoRepair" 1
    CreateShortcut "$SMPROGRAMS\\Trill.lnk" "$INSTDIR\\trill.exe"
    CreateShortcut "$DESKTOP\\Trill.lnk" "$INSTDIR\\trill.exe"
SectionEnd

Section "uninstall"
    SetDetailsPrint none
    Delete "$DESKTOP\\Trill.lnk"
    Delete "$SMPROGRAMS\\Trill.lnk"
    Delete "$INSTDIR\\libstdc++-6.dll"
    Delete "$INSTDIR\\libEGL.dll"
    Delete "$INSTDIR\\libGLESv2.dll"
    Delete "$INSTDIR\\libgcc_s_dw2-1.dll"
    Delete "$INSTDIR\\libwinpthread-1.dll"
    Delete "$INSTDIR\\ffmpeg.exe"
    Delete "$INSTDIR\\trill.exe"
    Delete "$INSTDIR\\uninstall.exe"
    RMDir $INSTDIR
    DeleteRegKey HKCU "<%text>$</%text>{REGPATH_UNINSTSUBKEY}"
SectionEnd
