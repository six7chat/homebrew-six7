class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.18"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.18"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.18/six7-0.7.18-arm64-apple-darwin.tar.gz"
      sha256 "f42339544f8e6b355ee7d80b5ca87eb0d83af338d9c0fd164d6227849035149d"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
