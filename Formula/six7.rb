class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.31"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.31"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.31/six7-0.7.31-arm64-apple-darwin.tar.gz"
      sha256 "780c0b92082ae7949e761966dac7bbd0ff231a6a5ca001fd3d414abf870ef58c"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
