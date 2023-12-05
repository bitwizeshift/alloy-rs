#include "boxer.h"
#include <string>
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <Windows.h>

namespace
{
#if defined(UNICODE)
   bool utf8ToUtf16(const char* utf8String, std::wstring& utf16String)
   {
      int count = MultiByteToWideChar(CP_UTF8, 0, utf8String, -1, nullptr, 0);
      if (count <= 0)
      {
         return false;
      }

      utf16String = std::wstring(static_cast<size_t>(count), L'\0');
      return MultiByteToWideChar(CP_UTF8, 0, utf8String, -1, &utf16String[0], count) > 0;
   }
#endif // defined(UNICODE)

   UINT getIcon(BoxerStyle style)
   {
      switch (style)
      {
      case InfoStyle:
         return MB_ICONINFORMATION;
      case WarningStyle:
         return MB_ICONWARNING;
      case ErrorStyle:
         return MB_ICONERROR;
      case QuestionStyle:
         return MB_ICONQUESTION;
      default:
         return MB_ICONINFORMATION;
      }
   }

   UINT getButtons(BoxerButtons buttons)
   {
      switch (buttons)
      {
      case OKButtons:
      case QuitButtons: // There is no 'Quit' button on Windows Buttons:(
         return MB_OK;
      case OKCancelButtons:
         return MB_OKCANCEL;
      case YesNoButtons:
         return MB_YESNO;
      default:
         return MB_OK;
      }
   }

   BoxerSelection getSelection(int response, BoxerButtons buttons)
   {
      switch (response)
      {
      case IDOK:
         return buttons == QuitButtons ? QuitSelection : OKSelection;
      case IDCANCEL:
         return CancelSelection;
      case IDYES:
         return YesSelection;
      case IDNO:
         return NoSelection;
      default:
         return NoneSelection;
      }
   }
} // namespace

extern "C"
BoxerSelection boxer_show(const char* message, const char* title, BoxerStyle style, BoxerButtons buttons)
{
   UINT flags = MB_TASKMODAL;

   flags |= getIcon(style);
   flags |= getButtons(buttons);

 #if defined(UNICODE)
   std::wstring wideMessage;
   std::wstring wideTitle;
   if (!utf8ToUtf16(message, wideMessage) || !utf8ToUtf16(title, wideTitle))
   {
      return ErrorSelection;
   }

   const WCHAR* messageArg = wideMessage.c_str();
   const WCHAR* titleArg = wideTitle.c_str();
#else
   const char* messageArg = message;
   const char* titleArg = title;
#endif

   return getSelection(MessageBox(nullptr, messageArg, titleArg, flags), buttons);
}
