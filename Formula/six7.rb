class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.24"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.24"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.24/six7-0.7.24-arm64-apple-darwin.tar.gz"
      sha256 "ba0d47273a00cb2de9f06947678bf617f29f0c3ba3ba66fbe804b8e1062a4de3"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
