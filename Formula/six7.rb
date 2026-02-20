class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.25"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.25"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.25/six7-0.7.25-arm64-apple-darwin.tar.gz"
      sha256 "096e921cdb84a0037c82433211f142a7bc3f4542043969f996fe26c07e7aeea0"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
