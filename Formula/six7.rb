class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.20"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.20"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.20/six7-0.7.20-arm64-apple-darwin.tar.gz"
      sha256 "e5b4ff33de20547850d75e92c8fe365bd8db5f567eda9782e78a1f70f760c595"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
